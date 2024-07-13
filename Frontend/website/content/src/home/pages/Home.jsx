import {createSignal} from "solid-js";
import {createStore} from "solid-js/store";

import Navbar from "../components/navbar";
import CurrentlyReading from "../components/currently_reading";
import YourLibraries from "../components/your_libraries";
import NewSummaries from "../components/new_summaries";
import RecommendedSummaries from "../components/recommended_summaries";
import YourSummaries from "../components/your_summaries";
import RecommendedShorts from "../components/recommended_shorts";
import Links from "../components/Links";
import Footer from "../components/footer";
import { getCookie } from "../../utils/cookies";

/** @typedef { import("../components/currently_reading").Summary } Summary */
/** @template T @typedef { import("solid-js/store").Store<T> } Store */

const Home = () => {
  // Fetch:
     // last 5 read summaries
     // last 5 read libraries
     // 15 New summaries
     // 15 Recommendations
     // First 6 Written summaries
     // 15 Recommended shorts

  /** @param {number} number_of_summaries */
  async function fetchLastReadSummaries(number_of_summaries) {
    let bearer_token = getCookie("Authorization")
    if (bearer_token == null) { bearer_token = "" };

    const response = await fetch("http://127.0.0.1:8000/summaries/last-read-summaries", {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        "Authorization": bearer_token,
      },
      body: JSON.stringify ({
        "number_of_summaries": number_of_summaries, 
      })
    });

    response.json().then((json) => setCurrentSummaries(json));
  }

  /** @param {number} number_of_summaries */
  async function fetchLastReadLibraries(number_of_summaries) {
    let bearer_token = getCookie("Authorization")
    if (bearer_token == null) { bearer_token = "" };

    const response = await fetch("http://127.0.0.1:8000/summaries/last-read-libraries", {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        "Authorization": bearer_token,
      },
      body: JSON.stringify ({
        "number_of_summaries": number_of_summaries, 
      })
    });

    response.json().then((json) => setYourLibraries(json));
  }

  /** @param {number} number_of_summaries
    * @param {string[]|null} genres */
  async function fetchNewSummaries(number_of_summaries, genres=null) {
    let bearer_token = getCookie("Authorization")
    if (bearer_token == null) { bearer_token = "" };

    const response = await fetch("http://127.0.0.1:8000/summaries/new-summaries", {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        "Authorization": bearer_token,
      },
      body: JSON.stringify ({
        "number_of_summaries": number_of_summaries, 
        "genres": genres,
      })
    });

    response.json().then((json) => setNewSummaries(json));
  }

  /** @param {number} number_of_summaries
    * @param {string[]|null} genres */
  async function fetchRecommendedSummaries(number_of_summaries, genres=null) {
    let bearer_token = getCookie("Authorization")
    if (bearer_token == null) { bearer_token = "" };

    const response = await fetch("http://127.0.0.1:8000/summaries/last-read", {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        "Authorization": bearer_token,
      },
      body: JSON.stringify ({
        "number_of_summaries": number_of_summaries, 
        "genres": genres,
      })
    });

    response.json().then((json) => setRecommendedSummaries(json));
  }

  /** @param {number} number_of_summaries */
  async function fetchYourSummaries(number_of_summaries) {
    let bearer_token = getCookie("Authorization")
    if (bearer_token == null) { bearer_token = "" };

    const response = await fetch("http://127.0.0.1:8000/summaries/last-read", {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        "Authorization": bearer_token,
      },
      body: JSON.stringify ({
        "number_of_summaries": number_of_summaries, 
      })
    });

    response.json().then((json) => setYourSummaries(json));
  }

  /** @param {number} number_of_summaries
    * @param {string[]|null} genres */
  async function fetchRecommendedShorts(number_of_summaries, genres=null) {
    let bearer_token = getCookie("Authorization")
    if (bearer_token == null) { bearer_token = "" };

    const response = await fetch("http://127.0.0.1:8000/summaries/recommended-shorts", {
      method: "POST",
      mode: "cors",
      headers: {
        "Content-Type": "application/json",
        "Authorization": bearer_token,
      },
      body: JSON.stringify ({
        "number_of_summaries": number_of_summaries,
        "genres": genres,
      })
    });

    response.json().then((json) => setRecommendedShorts(json));
  }

    
  const empty_summary = {
      author: { name: "", url: "" },
      book: { author: {name: "", url: ""}, title: "", url: "" },
      image_url: "",
      summary_url: ""
  };

  const [currentSummaries, setCurrentSummaries] = createStore([empty_summary]);
  const [newSummaries, setNewSummaries] = createStore([empty_summary]);
  const [recommendedSummaries, setRecommendedSummaries] = createStore([empty_summary]);
  const [recommendedShorts, setRecommendedShorts] = createStore([empty_summary]);
  const [yourLibraries, setYourLibraries] = createStore([empty_summary]);
  const [yourSummaries, setYourSummaries] = createStore([empty_summary]);

  return (
    <>
      <Navbar />
      <CurrentlyReading header="Currently Reading" summaries={currentSummaries}/>
      <YourLibraries libraries={YourLibraries} />
      <NewSummaries summaries={yourSummaries} />
      <RecommendedSummaries />
      <YourSummaries summaries={YourSummaries} />
      <RecommendedShorts />
      <Links />
      <Footer summary={{
        url: currentSummaries[0].summary_url,
        title: currentSummaries[0].book.title,
        image_url: currentSummaries[0].image_url
      }} />
    </>
  )
}

export default Home;
