/*
 * hyperscript helpers
 */

const text = string => {
  return document.createTextNode(string);
}

const div = (attributes, children) => {
  const element = document.createElement("div");
  for (const [ key, value ] of Object.entries(attributes))
    element.setAttribute(key, value);
  for (const child of children)
    element.appendChild(child)
  return element;
};


/*
 * groovebox
 */
const keys = {
  layout: {
    row1: Array.from("qwertyuiop"),
    row2: Array.from("asdfghjkl;"),
    row3: Array.from("zxcvbnm,./"),
  },
  role: {
    sequence: Array.from("qwertyuiasdfghjk"),
    control1: Array.from(".lo"),
    control2: Array.from("/;p"),
    navigation: Array.from("zxcvbnm,"),
  },
};

const newKey = key => {
  let role;
  if (keys.role.sequence.includes(key))
    role = "sequence";
  if (keys.role.navigation.includes(key))
    role = "navigation";
  if (keys.role.control1.includes(key))
    role = "control one";
  if (keys.role.control2.includes(key))
    role = "control two";
  return div({ class: "key centered " + role, "data-key": key }, [ text(key) ]);
};

document.body.appendChild(div({ class: "row centered" }, keys.layout.row1.map(newKey)));
document.body.appendChild(div({ class: "row centered" }, keys.layout.row2.map(newKey)));
document.body.appendChild(div({ class: "row centered" }, keys.layout.row3.map(newKey)));

const getKeyElement = key => {
  return document.querySelector(`[data-key="${key}"]`);
};

const updateKeyDown = (key, method) => {
  const element = getKeyElement(key);
  element && element.classList[method]("down");
};

document.addEventListener("keydown", event => updateKeyDown(event.key, "add"));
document.addEventListener("keyup", event => updateKeyDown(event.key, "remove"));

const updateState = ({ beat }) => {
  const cursor = keys.role.sequence[beat % 16];
  keys.role.sequence.forEach((key, index) => {
    getKeyElement(key).classList.toggle("lit", index === beat % keys.role.sequence.length);
  });
};

if (window.groovebox)
  (function mainLoop() {
    requestAnimationFrame(mainLoop);
    groovebox().then(updateState);
  })();
