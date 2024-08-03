import { For } from 'solid-js';
import SummaryItem from './summary_item';

/** @param {Array.<Props>} props */
const SummaryArray = (props) => {
  // add check for if array is fully left or fully right
  return (
    <>
      <div class="left-arrow"></div>
      <div class="summary-array">
        <For each={props.summary_data_array}>
          {(summary_data, i) => <SummaryItem summary_data={summary_data} />}
        </For>
      </div>
      <div class="right-arrow"></div>
    </>
  );
};

export default SummaryArray;
