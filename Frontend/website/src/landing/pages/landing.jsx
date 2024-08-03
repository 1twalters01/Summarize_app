import Navbar from '../components/navbar';
import CookiePopup from '../components/landing/CookiePopup';
import Hero from '../components/landing/Hero';
import Section1 from '../components/landing/Section1';
import Section2 from '../components/landing/Section2';
import Section3 from '../components/landing/Section3';
import Section4 from '../components/landing/Section4';
import Section5 from '../components/landing/Section5';
import Footer from '../components/landing/Footer';

const Landing = () => {
  return (
    <>
      <Navbar />
      <CookiePopup />
      <Hero />
      <Section1 />
      <Section2 />
      <Section3 />
      <Section4 />
      <Section5 />
      <Footer />
    </>
  );
};

export default Landing;
