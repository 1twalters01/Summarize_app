/** @template T @typedef { import('solid-js').Accessor<T> } Accessor */

/** @typedef {Object} Summary
  * @property {string} url - The summary's url
  * @property {string} image_url - The summary's image url
  * @property {string} title - The summary's title
*/

/** @typedef {Object} props
  * @property {Summary} summary - the summary in question
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
      <div class="summary">
        <a href={props.summary.url}>
          <img src={props.summary.image_url} />
          <p class="">{props.summary.title}</p>
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
