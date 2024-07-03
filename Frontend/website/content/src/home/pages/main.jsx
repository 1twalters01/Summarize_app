const Home = () => {
return (
<>
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

  
    <div class="currently-reading">
        <div class="main-header">
            <h1>Most Recent Reads</h1>
        </div>

        <div class="content">
            <div class="left">
                <div class="top">
                    <div class="subheader-1">
                        <h2 class="book-title">
                            <a href={book_url}>{book_title}</a>
                        </h2>
                        <h3 class="author-name">
                            <a href={author_url}>By {author_name}</a>
                        </h3>
                    </div>

                    <div class="subheader-2">
                        <h3 class="summary-by">Summary By</h3>
                        <h4 class="summary-author">{summary_author}</h3>
                    </div>
                </div>

                <div class="bottom">
                    <btn class="Read Now">Read Now</btn>
                </div>
            </div>

            <div class="right">
                <img href={book_img_url} />
            </div>
        </div>
        
        <div class="slider">
            <div class={is_slider1_active}></div>
            <div class={is_slider2_active}></div>
            <div class={is_slider3_active}></div>
            <div class={is_slider4_active}></div>
            <div class={is_slider5_active}></div>
        </div>
    </div>
</>
)
}

export default Home;
