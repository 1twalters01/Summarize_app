const YourLibraries = () => {
  return (
    <div class="your-libraries">
      <div class="top">
        <h3>Your Libraries</h3>

        <h3><a href="/libraries">View All</a></h3>
      </div>

      <div class="bottom">
        <LibraryArray library_data={library_data_array}/>
      </div>
    </div>
  )
};

export default YourLibraries;
