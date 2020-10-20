class GrooveboxKey {
  constructor(label, index) {
    this.index = index;
    this.element = document.createElement("div");
    this.element.innerText = label;
    this.element.classList.add("centered", "key");
  }

  onDown() {
    this.element.classList.add("down");
  }

  onUp() {
    this.element.classList.remove("down");
  }
}


/*
 * MAIN
 */

const keys = {};
Array.from("qwertyuiopasdfghjkl;zxcvbnm,./").map((label, index) => {
  const key = new GrooveboxKey(label, index);
  keys[label] = key;
  document.body.appendChild(key.element);
});
window.addEventListener("keyup", event => keys[event.key] && keys[event.key].onUp())
window.addEventListener("keydown", event => keys[event.key] && keys[event.key].onDown())
