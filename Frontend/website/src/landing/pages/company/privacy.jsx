import Navbar from '../../components/navbar';

const Privacy = () => {
  return (
    <>
      <Navbar />

      <h1>Privacy</h1>

      <a href="/privacy/pdf" target="_blank">
        pdf
      </a>
      <a href="/privacy/odf" download="Summarize Privacy Statement">
        odf
      </a>
      <a href="/privacy/docx" download="Summarize Privacy Statement">
        docx
      </a>
      <a href="/privacy/epub" download="Summarize Privacy Statement">
        epub
      </a>
      <a href="/privacy/latex" download="Summarize Privacy Statement">
        latex
      </a>
      <a href="/privacy/md" download="Summarize Privacy Statement">
        MD
      </a>
      <a href="/privacy/txt" download="Summarize Privacy Statement">
        txt
      </a>
    </>
  );
};

export default Privacy;
