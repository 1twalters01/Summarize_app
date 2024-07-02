/** @param {Array.<SummaryDataArray>} summary_data_array - Array of summary data */
const summary_array = (summary_data_array) => {
  // add check for if array is fully left or fully right
  return (
    <div class="left-arrow"></div>
    <div class="summary-array">
      <For each={summary_data_array}>{(summary_data, i) =>
        <SummaryItem summary_data={summary_data} />
      }</For>
    </div>
    <div class="right-arrow"></div>
  )
}

export default SummaryArray
