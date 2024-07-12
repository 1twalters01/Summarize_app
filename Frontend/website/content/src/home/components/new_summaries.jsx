import { For, createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import SummaryArray from "../../components/summary_array";

const NewSummaries = () => {
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
          <For each={genres}>{(genre) =>
            <button onClick={() => filterByGenre(genre)}>{genre}</button>
          }</For>
        </div>
      </div>

      <div class="bottom">
        <SummaryArray summary_data={summaryDataArray} />
      </div>
    </div>
  )
};

export default NewSummaries;
