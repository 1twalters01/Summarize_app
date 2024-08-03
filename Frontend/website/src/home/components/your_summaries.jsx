/** @template T @typedef { import("solid-js/store").Store<T> } Store */

/** @typedef {object} Summary */

/** @typedef {object} Props
 * @property {Store<Summary>} summaries
 */

/** @param {Props} props */
const YourSummaries = (props) => {
  return (
    <div class="your-summaries">
      <div class="top">
        <h3>Your Summaries</h3>

        <h3>
          <a href="/editor/all">View All</a>
        </h3>
      </div>

      <div class="bottom">
        <OwnSummaryArray own_summaary_data={own_summary_data_array} />
      </div>
    </div>
  );
};

export default YourSummaries;
