const Navbar = () => {
  return (
    <nav>
      <div class="left">
        <a href="/"><img src="" /></a>
      </div>
      <div class="center">
        <ThemeSlider />
      </div>
      <div class="right">
        <Hamburger />
      </div>
    </nav>
  )
};

export default Navbar;
