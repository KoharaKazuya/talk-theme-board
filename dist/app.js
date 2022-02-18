export default class App {
  constructor() {
    this.element = document.querySelector("#app");
  }

  start(themes) {
    this.themes = themes;
    this.index = 0;
    this.render();
  }

  prev() {
    if (!this.themes) return;
    this.index = Math.max(0, this.index - 1);
    this.render();
  }

  next() {
    if (!this.themes) return;
    this.index = Math.min(this.index + 1, this.themes.length - 1);
    this.render();
  }

  render() {
    if (!this.themes) return;

    const current = this.themes[this.index];
    if (!current) return;

    const main = current.slice(-1)[0];
    if (!main) return;
    const parents = current.slice(0, -1);

    const next = this.themes[this.index + 1];
    const nextMain = next?.slice(-1)[0]?.replace(/(<[^>]+>)/g, "");

    this.element.innerHTML = `
      <div class="current">
        <div class="breadcrumb">
          ${parents.map((p) => `<div>${p}</div>`).join("<div>&gt;</div>")}
        </div>
        <div class="main need-to-adjust-font-size">${main}</div>
      </div>
      ${
        nextMain
          ? `
      <div class="next">
        Next: ${nextMain}
      </div>
      `
          : ""
      }
    `;

    this.adjustFontSize();
  }

  adjustFontSize() {
    const container = document.scrollingElement;
    const target = this.element.querySelector(".need-to-adjust-font-size");

    function setFontSizeAndCheckScrollable(size) {
      target.style.fontSize = `${size}rem`;
      return (
        container.scrollWidth > container.clientWidth ||
        container.scrollHeight > container.clientHeight
      );
    }

    // binary search
    let min = 0.2,
      max = 5;
    while (max - min > 0.1) {
      const middle = (min + max) / 2;
      const scrollable = setFontSizeAndCheckScrollable(middle);
      if (scrollable) max = middle;
      else min = middle;
    }

    // adjust
    setFontSizeAndCheckScrollable(min);
    target.classList.remove("need-to-adjust-font-size");
  }
}
