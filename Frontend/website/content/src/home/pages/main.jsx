import {createSignal} from "solid-js";
import Footer from "../components/footer";

/** @template T @typedef { import('solid-js').Signal<T> } Signal */
/** @typedef { import("../components/footer").book } book */

const Home = () => {
  /** @type Signal<book> */
  const [currentBook, setCurrentBook] = createSignal({url: "", image_url: "", title: ""})
  return (
    <>
      <Navbar />
      <CurrentlyReading />
      <YourLibraries />
      <NewSummaries />
      <RecommendedForYou />
      <YourSummaries />
      <RecommendedShorts />
      <Links />
      <Footer book={currentBook} />
    </>
  )
}

export default Home;
