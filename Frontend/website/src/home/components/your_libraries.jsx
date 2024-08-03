import LibraryArray from '../../components/LibraryArray';

/** @template T @typedef { import("solid-js/store").Store<T> } Store */

/** @typedef {object} Library */

/** @typedef {object} Props
 * @property {Store<Library>} libraries
 */

/** @param {Props} props */
const YourLibraries = (props) => {
  return (
    <div class="your-libraries">
      <div class="top">
        <h3>Your Libraries</h3>

        <h3>
          <a href="/libraries">View All</a>
        </h3>
      </div>

      <div class="bottom">
        <LibraryArray libraries={props.libraries} />
      </div>
    </div>
  );
};

export default YourLibraries;
