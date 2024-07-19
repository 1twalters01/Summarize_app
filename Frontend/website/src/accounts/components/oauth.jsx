/** @typedef {Object} props
  * @property {string} googleText - google button text
  * @property {string} appleText - apple button text
  * @property {string} guestText - guest button text
*/

/** @param {props} props */
const Oauth2 = (props) => {
  return (
    <div class="oauth-btns">
      <button class="google">
        <img src="" />
        <p>{props.googleText}</p>
      </button>

      <button class="apple">
        <img src="" />
        <p>{props.appleText}</p>
      </button>

      <button class="guest">
        <img src="" />
        <p>{props.guestText}</p>
      </button>
    </div>
  )
};

export default Oauth2;
