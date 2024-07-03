const RecommendedForYou = () => {
  function filterByGenre(e) {
    // Do a post request to a new summaries api and then update summary_data_array
  }
  
  return (
    <div class="recommened-for-you">
      <div class="top">
        <div class="left">
          <For each={genres()}>{(genre) =>
              <btn onClick={filterByGenre}>{genre}</btn>
          }</For>
        </div>

        <div class="right">
          <h3>Recommended For You</h3>
        </div>
      </div>

      <div class="bottom">
        <SummaryArray summary_data={summary_data_array} />
      </div>
    <div />
  )
};

export default RecommendedForYou;
