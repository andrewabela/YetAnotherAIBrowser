const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.innerHTML = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});


// Handle URLs through Rust command
async function handleUrl(url) {
  try {
    const result = await invoke('handle_url_click', { url: url });
    console.log('URL handled successfully:', result);
    const linkMsgEl = document.getElementById('what-link-msg');
    if (linkMsgEl) {
      linkMsgEl.innerHTML = result;
    }
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
  const settingsBtn = document.getElementById('settings-btn');
  const settingsIframe = document.getElementById('settings-iframe');

  settingsBtn.addEventListener('click', () => {
    settingsIframe.style.display = 'block';
  });

  // Add a load listener to the iframe to ensure its content is ready
  settingsIframe.addEventListener('load', () => {
    try {
      const iframeDoc = settingsIframe.contentWindow.document;
      const settingsCloseBtn = iframeDoc.getElementById('close-settings-btn');

      if (settingsCloseBtn) {
        settingsCloseBtn.addEventListener('click', () => {
          settingsIframe.style.display = 'none';
        });
      } else {
        console.error('Close settings button not found in iframe.');
      }
    } catch (e) {
      console.error('Error accessing iframe content:', e);
    }
  });
});
