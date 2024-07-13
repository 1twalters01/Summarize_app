import LibraryArray from "../components/";

/** @template T @typedef { import("solid-js/store").Store<T> } Store */

/** @typedef {object} Props
  * @property {Store<Library>} libraries
  */

/** @param {Props} props */
const YourLibraries = (props) => {
  return (
    <div class="your-libraries">
      <div class="top">
        <h3>Your Libraries</h3>

        <h3><a href="/libraries">View All</a></h3>
      </div>

      <div class="bottom">
        <LibraryArray library_data={props.libraries}/>
      </div>
    </div>
  )
};

export default YourLibraries;
