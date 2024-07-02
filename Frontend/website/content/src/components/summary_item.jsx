/** @typedef {Object} SummaryData
  * @param {string} summary_url - The url to the suummary
  * @param {string} picture_url - The url to the picture of the book
  * @param {Book} book - The book the summary summarizes
  * @param {string} author - The user that wrote the summary
  * @param {string} author_url - The summary author's url
*/

/** typedef {Object} Book
  * title {string} - The title of the book
  * title_url {string} - The book's url
  * author {string} - The author of the book
  * author_url {string} - The book author's url
*/


/**
  * @param {SummaryData} book_data - The summary data required for a slide
*/
const SummaryItem = (summary_data) => {
  // Add loading state
  return (
    <div class="summary_item">
      <a href={summary_data.summary_url} ><img src={summary_data.picture_url} /></a>
      <a href={summary_data.book.title_url} class="book_title">{summary_data.book.title}</a>
      <a href={summary_data.book.author_url} class="book_author">{summary_data.book.author}</a>
      <a href={summary_data.summary_author_url} class="summary_author">{summary_data.author}</a>
    </div>
  )
}

export default SummaryItem;
