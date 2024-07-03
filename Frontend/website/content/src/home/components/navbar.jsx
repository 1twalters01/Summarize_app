const Navbar = () => {
  return (
    <nav>
      <div class="left">
        <a href="/"><img href="" /></a>
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
