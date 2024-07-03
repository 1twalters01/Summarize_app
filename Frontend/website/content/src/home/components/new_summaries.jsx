const NewSummaries = () => {
  function filterByGenre(e) {
    // Do a post request to a new summaries api and then update summary_data_array
  }
  
  return (
    <div class="new-summaries">
      <div class="top">
        <div class="left">
          <h3>My Summaries</h3>
        </div>

        <div class="right">
          <For each={genres()}>{(genre) =>
              <btn onClick={filterByGenre}>{genre}</btn>
          }</For>
        </div>
      </div>

      <div class="bottom">
        <SummaryArray summary_data={summary_data_array} />
      </div>
    <div />
  )
};

export default NewSummaries;
