using System;
using System.Collections.Generic;
using System.IO;
using System.Text.Json;
using Microsoft.UI.Xaml.Controls;
using Pyxillum.ViewModels;

namespace Pyxillum.Views
{
    public sealed partial class HardwareWizardDialog : ContentDialog
    {
        private readonly AppViewModel _viewModel;
        private int _currentStepIndex = 0;

        private record WizardStep(
            string Id,
            string Title,
            string Expected,
            Action DispatchAction
        );

        private record StepResult(
            string Id,
            string Title,
            string Result
        );

        private readonly WizardStep[] _steps;
        private readonly List<StepResult> _results = new();

        public HardwareWizardDialog(AppViewModel viewModel)
        {
            this.InitializeComponent();
            _viewModel = viewModel;

            _steps = new[]
            {
                new WizardStep("HW-01", "Power On Command", "The attached LED strip turns ON.",
                    () => _viewModel.Dispatch(new Pyxillum.Core.Action.SetPower(true))),
                new WizardStep("HW-02", "Solid Red Colour", "The LED strip displays pure RED across all addressable diodes.",
                    () => _viewModel.Dispatch(new Pyxillum.Core.Action.SetSolidColor(255, 0, 0))),
                new WizardStep("HW-03", "Solid Green Colour", "The LED strip displays pure GREEN across all addressable diodes.",
                    () => _viewModel.Dispatch(new Pyxillum.Core.Action.SetSolidColor(0, 255, 0))),
                new WizardStep("HW-04", "Solid Blue Colour", "The LED strip displays pure BLUE across all addressable diodes.",
                    () => _viewModel.Dispatch(new Pyxillum.Core.Action.SetSolidColor(0, 0, 255))),
                new WizardStep("HW-05", "Brightness 50%", "The LED strip dims visibly to ~50% luminosity without flicker.",
                    () => _viewModel.Dispatch(new Pyxillum.Core.Action.SetBrightness(128))),
                new WizardStep("HW-06", "Dynamic Rainbow Effect", "The LED strip executes a continuous smooth rainbow gradient cycle.",
                    () => _viewModel.Dispatch(new Pyxillum.Core.Action.SetEffect(1))),
                new WizardStep("HW-07", "Power Off Command", "The attached LED strip turns completely OFF.",
                    () => _viewModel.Dispatch(new Pyxillum.Core.Action.SetPower(false)))
            };

            this.PrimaryButtonClick += HardwareWizardDialog_PrimaryButtonClick;
            this.SecondaryButtonClick += HardwareWizardDialog_SecondaryButtonClick;

            LoadCurrentStep();
        }

        private void LoadCurrentStep()
        {
            if (_currentStepIndex >= _steps.Length) return;

            var step = _steps[_currentStepIndex];
            WizardProgressBar.Value = _currentStepIndex + 1;
            StepIdText.Text = $"Step {_currentStepIndex + 1} of {_steps.Length}: {step.Id}";
            StepTitleText.Text = step.Title;
            ExpectedVisualText.Text = step.Expected;

            if (_currentStepIndex == _steps.Length - 1)
            {
                PrimaryButtonText = "Pass (Complete)";
            }
            else
            {
                PrimaryButtonText = "Pass (Next)";
            }

            // Execute command for current step
            step.DispatchAction();
        }

        private void HardwareWizardDialog_PrimaryButtonClick(ContentDialog sender, ContentDialogButtonClickEventArgs args)
        {
            var step = _steps[_currentStepIndex];
            _results.Add(new StepResult(step.Id, step.Title, "PASS"));

            if (_currentStepIndex < _steps.Length - 1)
            {
                args.Cancel = true;
                _currentStepIndex++;
                LoadCurrentStep();
            }
            else
            {
                SaveResults();
            }
        }

        private void HardwareWizardDialog_SecondaryButtonClick(ContentDialog sender, ContentDialogButtonClickEventArgs args)
        {
            var step = _steps[_currentStepIndex];
            _results.Add(new StepResult(step.Id, step.Title, "FAIL"));

            if (_currentStepIndex < _steps.Length - 1)
            {
                args.Cancel = true;
                _currentStepIndex++;
                LoadCurrentStep();
            }
            else
            {
                SaveResults();
            }
        }

        private void SaveResults()
        {
            try
            {
                var doc = new
                {
                    timestamp = DateTime.UtcNow.ToString("o"),
                    target = _viewModel.Connection.Address ?? "SIM:00:11:22:33:44:55",
                    simulated = _viewModel.IsPreviewMode,
                    results = _results
                };

                string json = JsonSerializer.Serialize(doc, new JsonSerializerOptions { WriteIndented = true });
                string resultsPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "tests", "hardware", "RESULTS.json");
                
                // Fallback to relative workspace path if running in dev environment
                if (!Directory.Exists(Path.GetDirectoryName(resultsPath)))
                {
                    resultsPath = Path.GetFullPath("tests/hardware/RESULTS.json");
                }

                if (Directory.Exists(Path.GetDirectoryName(resultsPath)))
                {
                    File.WriteAllText(resultsPath, json);
                }
            }
            catch (Exception ex)
            {
                Diagnostics.Logger.Error("[HardwareWizard] Failed to persist verification results", ex);
            }
        }
    }
}
