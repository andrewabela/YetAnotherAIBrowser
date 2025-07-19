const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

// Function to show save status feedback in the settings iframe
function showSaveStatus(iframeDoc, message, isError = false) {
  // Remove any existing status message
  let existingStatus = iframeDoc.getElementById('save-status');
  if (existingStatus) {
    existingStatus.remove();
  }

  // Create new status message
  const statusDiv = iframeDoc.createElement('div');
  statusDiv.id = 'save-status';
  statusDiv.textContent = message;
  statusDiv.style.cssText = `
    position: fixed;
    bottom: 10px;
    left: 10px;
    padding: 8px 12px;
    border-radius: 4px;
    font-size: 12px;
    font-weight: bold;
    z-index: 1000;
    transition: opacity 0.3s ease;
    ${isError ?
      'background-color: #fee; color: #c33; border: 1px solid #fcc;' :
      'background-color: #efe; color: #383; border: 1px solid #cfc;'
    }
  `;

  // Add to iframe document
  iframeDoc.body.appendChild(statusDiv);

  // Auto-remove after 2 seconds
  setTimeout(() => {
    if (statusDiv && statusDiv.parentNode) {
      statusDiv.style.opacity = '0';
      setTimeout(() => {
        if (statusDiv && statusDiv.parentNode) {
          statusDiv.remove();
        }
      }, 300);
    }
  }, 2000);
}

// Handle URLs through Rust command
async function handleUrl(url) {
  try {
    const result = await invoke('handle_url_click', { url: url });
    console.log('URL handled successfully:', result);
    // alert("bob");
    // const linkMsgEl = document.getElementById('what-link-msg');
    // if (linkMsgEl) {
    //   linkMsgEl.innerHTML = result;
    // }
    const content_iframe = document.getElementById('content-iframe');
    content_iframe.srcdoc = result;
    return result;
  } catch (error) {
    console.error('Error handling URL:', error);
    throw error;
  }
}

// Intercept all link clicks and prevent default browser behavior
document.addEventListener('click', function (e) {
  // Check if the clicked element is a link or has a parent link
  let link = e.target.closest('a');
  if (link && link.href) {
    e.preventDefault();
    e.stopPropagation();
    handleUrl(link.href);
  }
});

// Also intercept middle mouse and right mouse clicks on links
document.addEventListener('auxclick', function (e) {
  let link = e.target.closest('a');
  if (link && link.href && (e.button === 1 || e.button === 2)) {
    e.preventDefault();
    handleUrl(link.href);
  }
});

// Handle programmatic URL opens and form submissions
document.addEventListener('DOMContentLoaded', function () {
  // Intercept programmatic window.open
  const originalOpen = window.open;
  window.open = function (url, target, features) {
    if (url) {
      handleUrl(url);
      return null;
    }
    return originalOpen.call(window, url, target, features);
  };

  // Intercept external form submissions
  document.addEventListener('submit', function (e) {
    const form = e.target;
    const url = new URL(form.action, window.location.href);
    if (url.origin !== window.location.origin) {
      e.preventDefault();
      const formData = new FormData(form);
      const params = new URLSearchParams(formData);
      handleUrl(url.toString() + '?' + params.toString());
    }
  });
});


document.addEventListener('DOMContentLoaded', () => {
  console.log('Main.js DOMContentLoaded');
  const settingsBtn = document.getElementById('settings-btn');
  const settingsIframe = document.getElementById('settings-iframe');

  console.log('Settings button:', settingsBtn);
  console.log('Settings iframe:', settingsIframe);

  settingsBtn.addEventListener('click', () => {
    console.log('Settings button clicked');
    settingsIframe.style.display = 'block';
  });

  // Add a load listener to the iframe to ensure its content is ready
  settingsIframe.addEventListener('load', async () => {
    console.log('Settings iframe loaded');
    try {
      const iframeDoc = settingsIframe.contentWindow.document;
      console.log('Iframe document:', iframeDoc);

      const settingsCloseBtn = iframeDoc.getElementById('close-settings-btn');
      console.log('Close button found:', settingsCloseBtn);

      if (settingsCloseBtn) {
        settingsCloseBtn.addEventListener('click', () => {
          console.log('Close button clicked');
          settingsIframe.style.display = 'none';
        });
      } else {
        console.error('Close settings button not found in iframe.');
      }

      // Load LLM providers and populate the dropdown in the iframe
      try {
        console.log('Loading LLM providers from main window');
        showSaveStatus(iframeDoc, 'Loading providers...', false);

        const providers = await invoke('get_all_llm_providers');
        console.log('Providers loaded:', providers);

        const llmProviderSelect = iframeDoc.getElementById('llm-provider');
        console.log('LLM provider select element:', llmProviderSelect);

        if (llmProviderSelect) {
          llmProviderSelect.innerHTML = ''; // Clear existing options

          // Add a default "Select provider" option
          const defaultOption = iframeDoc.createElement('option');
          defaultOption.value = '';
          defaultOption.textContent = 'Select a provider...';
          llmProviderSelect.appendChild(defaultOption);

          providers.forEach(provider => {
            const option = iframeDoc.createElement('option');
            option.value = provider[0]; // First element is the ID
            option.textContent = provider[1]; // Second element is the name
            llmProviderSelect.appendChild(option);
            console.log('Added provider:', provider[1]);
          });

          console.log('LLM providers loaded successfully in iframe');
          console.log('Final select HTML:', llmProviderSelect.outerHTML);
          showSaveStatus(iframeDoc, 'Providers loaded', false);
        } else {
          console.error('llm-provider select element not found in iframe');
          showSaveStatus(iframeDoc, 'Error: Provider select not found', true);
        }
      } catch (error) {
        console.error('Error loading LLM providers in main window:', error);
        showSaveStatus(iframeDoc, 'Error loading providers', true);
      }

      // Load current settings
      console.log('Loading current settings');
      try {
        console.log('Loading saved settings');
        showSaveStatus(iframeDoc, 'Loading settings...', false);

        const [provider, endpoint, apiKey, model] = await Promise.all([
          invoke('get_current_llm_provider'),
          invoke('get_llm_endpoint'),
          invoke('get_llm_key'),
          invoke('get_llm_model')
        ]);

        // Set the saved values
        const llmProviderSelect = iframeDoc.getElementById('llm-provider');
        const llmEndpointInput = iframeDoc.getElementById('llm-provider-endpoint');
        const llmKeyInput = iframeDoc.getElementById('llm-provider-key');
        const llmModelInput = iframeDoc.getElementById('llm-provider-model');

        if (provider && llmProviderSelect) {
          llmProviderSelect.value = provider;
          console.log('Current provider set in select:', provider);
        }
        if (endpoint && llmEndpointInput) {
          llmEndpointInput.value = endpoint;
          console.log('LLM endpoint set in input:', endpoint);
        }
        if (apiKey && llmKeyInput) {
          llmKeyInput.value = apiKey;
          console.log('LLM API key set in input');
        }
        if (model && llmModelInput) {
          llmModelInput.value = model;
          console.log('LLM model set in input:', model);
        }

        console.log('Settings loaded:', { provider, endpoint, apiKey: apiKey ? '***' : '', model });
        showSaveStatus(iframeDoc, 'Settings loaded', false);
      } catch (error) {
        console.error('Error loading settings:', error);
        showSaveStatus(iframeDoc, 'Error loading settings', true);
      }

      // Add event listeners to save settings when changed
      try {
        const llmProviderSelect = iframeDoc.getElementById('llm-provider');
        if (llmProviderSelect) {
          llmProviderSelect.addEventListener('change', async (e) => {
            try {
              const selectedProvider = e.target.value;

              // Save the selected provider
              await invoke('set_current_llm_provider', { provider: selectedProvider });
              console.log('LLM provider saved:', selectedProvider);
              showSaveStatus(iframeDoc, 'Provider saved');

              // Load and apply defaults for the selected provider (if not empty selection)
              if (selectedProvider && selectedProvider !== '') {
                try {
                  showSaveStatus(iframeDoc, 'Loading provider defaults...', false);
                  const [defaultEndpoint, defaultApiKey, defaultModel] = await invoke('get_provider_defaults', { providerId: selectedProvider });

                  // Get form elements
                  const llmEndpointInput = iframeDoc.getElementById('llm-provider-endpoint');
                  const llmKeyInput = iframeDoc.getElementById('llm-provider-key');
                  const llmModelInput = iframeDoc.getElementById('llm-provider-model');

                  // Set defaults and save them
                  if (llmEndpointInput && defaultEndpoint) {
                    llmEndpointInput.value = defaultEndpoint;
                    await invoke('set_llm_endpoint', { endpoint: defaultEndpoint });
                    console.log('Default endpoint loaded:', defaultEndpoint);
                  }

                  if (llmKeyInput && defaultApiKey) {
                    llmKeyInput.value = defaultApiKey;
                    await invoke('set_llm_key', { key: defaultApiKey });
                    console.log('Default API key loaded');
                  }

                  if (llmModelInput && defaultModel) {
                    llmModelInput.value = defaultModel;
                    await invoke('set_llm_model', { model: defaultModel });
                    console.log('Default model loaded:', defaultModel);
                  }

                  showSaveStatus(iframeDoc, 'Provider defaults loaded');
                } catch (defaultsError) {
                  console.error('Error loading provider defaults:', defaultsError);
                  showSaveStatus(iframeDoc, 'Provider saved, but defaults failed to load', true);
                }
              }
            } catch (error) {
              console.error('Error saving LLM provider:', error);
              showSaveStatus(iframeDoc, 'Error saving provider', true);
            }
          });
        }

        const llmEndpointInput = iframeDoc.getElementById('llm-provider-endpoint');
        if (llmEndpointInput) {
          // Save on input change (real-time)
          let endpointTimeout;
          llmEndpointInput.addEventListener('input', async (e) => {
            clearTimeout(endpointTimeout);
            endpointTimeout = setTimeout(async () => {
              try {
                await invoke('set_llm_endpoint', { endpoint: e.target.value });
                console.log('LLM endpoint saved:', e.target.value);
                showSaveStatus(iframeDoc, 'Endpoint saved');
              } catch (error) {
                console.error('Error saving LLM endpoint:', error);
                showSaveStatus(iframeDoc, 'Error saving endpoint', true);
              }
            }, 500); // Debounce for 500ms
          });

          // Also save on blur for reliability
          llmEndpointInput.addEventListener('blur', async (e) => {
            clearTimeout(endpointTimeout);
            try {
              await invoke('set_llm_endpoint', { endpoint: e.target.value });
              console.log('LLM endpoint saved (blur):', e.target.value);
              showSaveStatus(iframeDoc, 'Endpoint saved');
            } catch (error) {
              console.error('Error saving LLM endpoint:', error);
              showSaveStatus(iframeDoc, 'Error saving endpoint', true);
            }
          });
        }

        const llmKeyInput = iframeDoc.getElementById('llm-provider-key');
        if (llmKeyInput) {
          // Save on input change (real-time)
          let keyTimeout;
          llmKeyInput.addEventListener('input', async (e) => {
            clearTimeout(keyTimeout);
            keyTimeout = setTimeout(async () => {
              try {
                await invoke('set_llm_key', { key: e.target.value });
                console.log('LLM API key saved');
                showSaveStatus(iframeDoc, 'API key saved');
              } catch (error) {
                console.error('Error saving LLM API key:', error);
                showSaveStatus(iframeDoc, 'Error saving API key', true);
              }
            }, 500); // Debounce for 500ms
          });

          // Also save on blur for reliability
          llmKeyInput.addEventListener('blur', async (e) => {
            clearTimeout(keyTimeout);
            try {
              await invoke('set_llm_key', { key: e.target.value });
              console.log('LLM API key saved (blur)');
              showSaveStatus(iframeDoc, 'API key saved');
            } catch (error) {
              console.error('Error saving LLM API key:', error);
              showSaveStatus(iframeDoc, 'Error saving API key', true);
            }
          });
        }

        const llmModelInput = iframeDoc.getElementById('llm-provider-model');
        if (llmModelInput) {
          // Save on input change (real-time)
          let modelTimeout;
          llmModelInput.addEventListener('input', async (e) => {
            clearTimeout(modelTimeout);
            modelTimeout = setTimeout(async () => {
              try {
                await invoke('set_llm_model', { model: e.target.value });
                console.log('LLM model saved:', e.target.value);
                showSaveStatus(iframeDoc, 'Model saved');
              } catch (error) {
                console.error('Error saving LLM model:', error);
                showSaveStatus(iframeDoc, 'Error saving model', true);
              }
            }, 500); // Debounce for 500ms
          });

          // Also save on blur for reliability
          llmModelInput.addEventListener('blur', async (e) => {
            clearTimeout(modelTimeout);
            try {
              await invoke('set_llm_model', { model: e.target.value });
              console.log('LLM model saved (blur):', e.target.value);
              showSaveStatus(iframeDoc, 'Model saved');
            } catch (error) {
              console.error('Error saving LLM model:', error);
              showSaveStatus(iframeDoc, 'Error saving model', true);
            }
          });
        }
      } catch (error) {
        console.error('Error setting up event listeners:', error);
      }

    } catch (e) {
      console.error('Error accessing iframe content:', e);
    }
  });
});
