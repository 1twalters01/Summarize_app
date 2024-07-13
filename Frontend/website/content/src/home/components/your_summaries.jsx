// Placeholder things
const YourSummaries = () => {
  return (
    <div class="your-summaries">
      <div class="top">
        <h3>Your Summaries</h3>

        <h3><a href="/editor/all">View All</a></h3>
      </div>

      <div class="bottom">
        <OwnSummaryArray own_summaary_data={own_summary_data_array}/>
      </div>
    </div>
  )
};

export default YourSummaries;
