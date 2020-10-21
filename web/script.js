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
  keys[index] = key;
  document.body.appendChild(key.element);
});
document.addEventListener("keyup", event => keys[event.key] && keys[event.key].onUp());
document.addEventListener("keydown", event => keys[event.key] && keys[event.key].onDown());

(function loop() {
  requestAnimationFrame(loop);
  groovebox().then(({ measure, beat }) => {
    Object.values(keys).forEach(key => {
      key.element.classList.remove("beat");
      key.element.classList.toggle("extra", key.index >= measure);
    });
    keys[beat % measure].element.classList.add("beat");
  });
})();
