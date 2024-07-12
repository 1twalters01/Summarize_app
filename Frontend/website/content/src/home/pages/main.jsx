import {createSignal} from "solid-js";

import Navbar from "../components/navbar";
import CurrentlyReading from "../components/currently_reading";
import YourLibraries from "../components/your_libraries";
import NewSummaries from "../components/new_summaries";
import RecommendedForYou from "../components/recommended_for_you";
import YourSummaries from "../components/your_summaries";
import RecommendedShorts from "../components/recommended_shorts";
import Links from "../components/Links";
import Footer from "../components/footer";

/** @template T @typedef { import('solid-js').Signal<T> } Signal */
/** @typedef { import("../components/currently_reading").Summary } Summary */

const Home = () => {
  const empty_summary = {
      author: {
          name: "",
          url: ""
      },
      book: {
          author: {name: "", url: ""},
          title: "",
          url: ""
      },
      image_url: "",
      summary_url: ""
  };

  /** @type Signal<Summary> */
  const [currentSummary, setCurrentSummary] = createSignal(empty_summary);

  return (
    <>
      <Navbar />
      <CurrentlyReading header="Currently Reading" summary={currentSummary}/>
      <YourLibraries />
      <NewSummaries />
      <RecommendedForYou />
      <YourSummaries />
      <RecommendedShorts />
      <Links />
      <Footer summary={{
        url: currentSummary().summary_url,
        title: currentSummary().book.title,
        image_url: currentSummary().image_url}} />
    </>
  )
}

export default Home;
