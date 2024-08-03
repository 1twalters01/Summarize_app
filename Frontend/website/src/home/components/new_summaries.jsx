import { For, createSignal } from 'solid-js';
import { createStore } from 'solid-js/store';
import SummaryArray from '../../components/summary_array';

/** @template T @typedef { import("solid-js/store").Store<T> } Store */

/** @typedef {object} Summary */

/** @typedef {object} Props
 * @property {Store<Summary>} summaries
 */

/** @param {Props} props */
const NewSummaries = (props) => {
  const [genres, setGenres] = createStore([]);
  const [summaryDataArray, setSummaryDataArray] = createStore([]);

  /** @param {string} genre - The selected genre */
  function filterByGenre(genre) {
    // Do a post request to a new summaries api and then update summary_data_array
  }

  return (
    <div class="new-summaries">
      <div class="top">
        <div class="left">
          <h3>My Summaries</h3>
        </div>

        <div class="right">
          <For each={genres}>
            {(genre) => (
              <button onClick={() => filterByGenre(genre)}>{genre}</button>
            )}
          </For>
        </div>
      </div>

      <div class="bottom">
        <SummaryArray summary_data={props.summaries} />
      </div>
    </div>
  );
};

export default NewSummaries;
