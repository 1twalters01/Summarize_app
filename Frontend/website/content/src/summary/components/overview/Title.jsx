/** @typedef {object} Author
  * @property {string} name
  */

/** @typedef {object} Book
  * @property {string} title
  * @property {string} series
  * @property {string} publisher
  * @property {Author} author
  */

/** @typedef {object} Props
  * @property {Book} book
  */

/** @param {Props} props */
const Title = (props) => {
  return (
    <div class="title">
      <h1>{props.book.title} By {props.book.author.name}</h1>
      <h2>{props.book.series}</h2>
      <h3>{props.book.publisher}</h3>
    </div>
  )
}

export default Title;
