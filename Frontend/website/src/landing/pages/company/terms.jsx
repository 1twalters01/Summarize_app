import Navbar from "../../components/navbar";

const Terms = () => {
  return (
    <>
      <Navbar />

      <h1>Terms</h1>

      <a href="/terms/pdf" target="_blank">pdf</a>
      <a href="/terms/odf" download="Summarize Terms Statement">odf</a>
      <a href="/terms/docx" download="Summarize Terms Statement">docx</a>
      <a href="/terms/epub" download="Summarize Terms Statement">epub</a>
      <a href="/terms/latex" download="Summarize Terms Statement">latex</a>
      <a href="/terms/md" download="Summarize Terms Statement">MD</a>
      <a href="/terms/txt" download="Summarize Terms Statement">txt</a>
    </>
  )
};

export default Terms;

