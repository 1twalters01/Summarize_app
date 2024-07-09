/** @typedef {Object} props
  * @property {String} subheader - go to next screen
*/

/** @param {props} props */
const Headers = (props) => {
  return (
    <div>
      <h1>Read Smarter, Not Harder</h1>
      <h2>{props.subheader}</h2>
    </div>
  )
};

export default Headers;
