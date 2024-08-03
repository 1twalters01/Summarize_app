import { createSignal } from 'solid-js';

/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */
/** @template T @typedef { import("solid-js/store").Store<T> } Store */

/** @typedef {Object} Summary
 * @property {string} summary_url - The url to the suummary
 * @property {string} image_url - The url to the picture of the book
 * @property {Book} book - The book the summary summarizes
 * @property {Author} author - The user that wrote the summary
 */

/** @typedef {Object} Book
 * @property {string} title - The title of the book
 * @property {string} url - The book's url
 * @property {Author} author - The author of the book
 */

/** @typedef {Object} Author
 * @property {string} name - The author
 * @property {string} url - The author's url
 */

/** @typedef {Object} Props
 * @property {string} header - The section's header
 * @property {Store<Summary[]>} summaries - The summaries for the books
 */

/** @param {Props} props */
const CurrentlyReading = (props) => {
  const [slider1State, setSlider1State] = createSignal(true);
  const [slider2State, setSlider2State] = createSignal(false);
  const [slider3State, setSlider3State] = createSignal(false);
  const [slider4State, setSlider4State] = createSignal(false);
  const [slider5State, setSlider5State] = createSignal(false);

  /** @param {number} slider_number - the slider in question */
  function setSliderState(slider_number) {
    setSlider1State(false);
    setSlider2State(false);
    setSlider3State(false);
    setSlider4State(false);
    setSlider5State(false);

    switch (slider_number) {
      case 1:
        setSlider1State(true);
        break;
      case 2:
        setSlider2State(true);
        break;
      case 3:
        setSlider3State(true);
        break;
      case 4:
        setSlider4State(true);
        break;
      case 5:
        setSlider1State(true);
        break;
      default:
        break;
    }
  }

  return (
    <div class={props.header}>
      <div class="main-header">
        <h1>{props.header}</h1>
      </div>

      <div class="content">
        <div class="left">
          <div class="top">
            <div class="subheader-1">
              <h2 class="book-title">
                <a href={props.summaries[0].book.url}>
                  {props.summaries[0].book.title}
                </a>
              </h2>
              <h3 class="author-name">
                <a href={props.summaries[0].book.author.url}>
                  By {props.summaries[0].book.author.name}
                </a>
              </h3>
            </div>

            <div class="subheader-2">
              <h3 class="summary-by">Summary By</h3>
              <h4 class="summary-author">{props.summaries[0].author.name}</h4>
            </div>
          </div>

          <div class="bottom">
            <button class="Read Now">Read Now</button>
          </div>
        </div>

        <div class="right">
          <img src={props.summaries[0].image_url} />
        </div>
      </div>

      <div class="slider">
        <div
          class={'slider-' + slider1State()}
          onClick={() => setSliderState(1)}
        ></div>
        <div
          class={'slider-' + slider2State()}
          onClick={() => setSliderState(2)}
        ></div>
        <div
          class={'slider-' + slider3State()}
          onClick={() => setSliderState(3)}
        ></div>
        <div
          class={'slider-' + slider4State()}
          onClick={() => setSliderState(4)}
        ></div>
        <div
          class={'slider-' + slider5State()}
          onClick={() => setSliderState(5)}
        ></div>
      </div>
    </div>
  );
};

export default CurrentlyReading;
