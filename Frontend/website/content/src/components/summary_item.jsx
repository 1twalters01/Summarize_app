/** @typedef {Object} SummaryData
  * @param {string} picture_url - The url to the picture of the book
  * @param {Book} book - The book the summary summarizes
  * @param {string} author - The user that wrote the summary
*/


/** typedef {Object} Book
  * title {string} - The title of the book
  * author {string} - The author of the book
*/

/**
  * @param {SummaryData} book_data - The summary data required for a slide
*/
const summary_item = (summary_data) => {
  return (
    <div class="summary_item">
      <img src={summary_data.picture_url} />
      <p class="book_title">{summary_data.book.title}</p>
      <p class="book_author">{summary_data.book.author}</p>
      <p class="summary_author">{summary_data.author}</p>
    </div>
  )
}

export default summary_item;
