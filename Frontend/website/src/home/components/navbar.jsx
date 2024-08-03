import ThemeSlider from './navbar/ThemeSlider';
import Search from './navbar/Search';
import Hamburger from './navbar/Hamburger';

const Navbar = () => {
  return (
    <nav>
      <div class="left">
        <a href="/">
          <img src="/favicon.ico" />
        </a>
      </div>
      <div class="center-left">
        <ThemeSlider />
      </div>
      <div class="center-right">
        <Search />
      </div>
      <div class="right">
        <Hamburger />
      </div>
    </nav>
  );
};

export default Navbar;
