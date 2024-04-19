package uk.summarize.summarize

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button
import android.widget.TextView
import uk.summarize.summarize.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {

    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // Example of a call to a native method
        binding.sampleText.text = stringFromJNI()

        val nextButton = findViewById<Button>(R.id.next_btn)
        val prevButton = findViewById<Button>(R.id.prev_btn)
    }

    /**
     * A native method that is implemented by the 'summarize' native library,
     * which is packaged with this application.
     */
    external fun stringFromJNI(): String

    external fun nextBtnFromJNI(): String


    external fun prevBtnFromJNI(): String

    companion object {
        // Used to load the 'summarize' library on application startup.
        init {
            System.loadLibrary("summarize")
        }
    }
}