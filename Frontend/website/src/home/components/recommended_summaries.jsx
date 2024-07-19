import { For } from "solid-js";
import { createStore } from "solid-js/store";

/** @template T @typedef { import("solid-js/store").Store<T> } Store */

/** @typedef {object} Summary */

/** @typedef {object} Props
  * @property {Store<Summary>} summaries
  */

/** @param {Props} props */
const RecommendedSummaries = (props) => {
  const [genres, setGenres] = createStore([]);

  function filterByGenre(e) {
    // Do a post request to a new summaries api and then update summary_data_array
  }
  
  return (
    <div class="recommened-for-you">
      <div class="top">
        <div class="left">
          <For each={genres}>{(genre) =>
              <button onClick={filterByGenre}>{genre}</button>
          }</For>
        </div>

        <div class="right">
          <h3>Recommended For You</h3>
        </div>
      </div>

      <div class="bottom">
        <SummaryArray summary_data={props.summaries} />
      </div>
    </div>
  )
};

export default RecommendedSummaries;
