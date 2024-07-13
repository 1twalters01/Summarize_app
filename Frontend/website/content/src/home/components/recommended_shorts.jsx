import { For } from "solid-js";
import { createStore } from "solid-js/store"

/** @template T @typedef { import("solid-js/store").Store<T> } Store */

/** @typedef {object} Summary */

/** @typedef {object} Props
  * @property {Store<Summary>} summaries
  */

/** @param {Props} props */
const RecommendedShorts = (props) => {
  function filterByGenre(e) {
    // Do a post request to a new summaries api and then update summary_data_array
  }
  
  const [genres, setGenres] = createStore([]);
  return (
    <div class="recommended-shorts">
      <div class="top">
        <div class="left">
          <h3>Recommended Shorts</h3>
        </div>

        <div class="right">
          <For each={genres}>{(genre) =>
              <button onClick={filterByGenre}>{genre}</button>
          }</For>
        </div>
      </div>

      <div class="bottom">
        <SummaryArray summary_data={props.summaries} />
      </div>
    </div>
  )
};

export default RecommendedShorts;
