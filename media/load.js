// This file should not be changed.

function createStyleElement(stylename) {
  // <link rel="stylesheet" type="text/css" href="mystyles.css" media="screen" />

  let styleEl = document.createElement("link");
  styleEl.setAttribute("rel", "stylesheet");
  styleEl.setAttribute("type", "text/css");
  styleEl.setAttribute("media", "screen");
  styleEl.setAttribute("href", `${stylename}`);

  return styleEl;
}

let head = document.getElementsByTagName("head")[0];

head.appendChild(createStyleElement("global.css"));
