/** @typedef {Object} book
  * @property {string} url - The book's url
  * @property {string} image_url - The book's image url
  * @property {string} title - The book's title
*/

/** @typedef {Object} props
  * @property {book} book - the book in question
*/

/** @param {props} props */
const Footer = (props) => {
  return (
    <footer>
      <div class="home">
        <a href="/home">
          <img src="" />
          <p class="">Home</p>
        </a>
      </div>
      <div class="libraries">
        <a href="/libraries">
          <img src="" />
          <p class="">Libraries</p>
        </a>
      </div>
      <div class="book">
        <a href={props.book.url}>
          <img src={props.book.image_url} />
          <p class="">{props.book.title}</p>
        </a>
      </div>
      <div class="search">
        <a href="/search">
          <img src="" />
          <p class="">Search</p>
        </a>
      </div>
      <div class="editor">
        <a href="/editor">
          <img src="" />
          <p class="">Write</p>
        </a>
      </div>
    </footer>
  )
}

export default Footer;
