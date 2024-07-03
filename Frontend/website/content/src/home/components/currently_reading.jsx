const CurrentlyReading = (summary) = {
  return (
    <div class="currently-reading">
      <div class="main-header">
        <h1>Currently Reading</h1>
      </div>

      <div class="content">
        <div class="left">
          <div class="top">
            <div class="subheader-1">
              <h2 class="book-title">
                <a href={summary.book.url}>{summary.book.title}</a>
              </h2>
              <h3 class="author-name">
                <a href={summary.book.author.url}>By {summary.book.author.name}</a>
              </h3>
            </div>

            <div class="subheader-2">
              <h3 class="summary-by">Summary By</h3>
              <h4 class="summary-author">{summary.author}</h3>
            </div>
          </div>

          <div class="bottom">
            <btn class="Read Now">Read Now</btn>
          </div>
        </div>

        <div class="right">
          <img href={summary.book.img_url} />
        </div>
      </div>
        
      <div class="slider">
        <div class={is_slider1_active}></div>
        <div class={is_slider2_active}></div>
        <div class={is_slider3_active}></div>
        <div class={is_slider4_active}></div>
        <div class={is_slider5_active}></div>
      </div>
    </div>
  )
}

export default CurrentlyReading;
