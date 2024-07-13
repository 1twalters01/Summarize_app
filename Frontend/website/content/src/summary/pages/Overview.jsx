import { createSignal } from "solid-js";

import Navbar from "../../home/components/navbar";
import Title from "../overview/components/Title";
import Main from "../overview/components/Main";
import KeySummary from "../overview/components/KeySummary";
import MoreFromSummaryAuthor from "../overview/components/SummaryAuthor";
import MoreFromBookAuthor from "../overview/components/BookAuthor";
import Similar from "../overview/components/Similar";
import Comments from "../overview/components/Comments";
import Footer from "../../home/components/footer";

const Overview = () => {
  const [currentSummary, setCurrentSummary] = createSignal();

  return (
    <>
      <Navbar />
      <Title />
      <Main />
      <KeySummary />
      <MoreFromSummaryAuthor />
      <MoreFromBookAuthor />
      <Similar />
      <Comments />
      <Footer summary={{
        url: currentSummary.summary_url,
        title: currentSummary.book.title,
        image_url: currentSummary.image_url
      }} />
    </>
  )
}

export default Overview;
