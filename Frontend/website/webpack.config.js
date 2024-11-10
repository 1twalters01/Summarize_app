import { dirname, join } from "path";
import { fileURLToPath } from "url";
import Dotenv from 'dotenv-webpack';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const data = {
  entry: "./src/index.jsx",
  output: {
    path: join(__dirname, "dist/main/javascript"),
    filename: "bundle.js",
  },
  // mode: "development",
  mode: "production",
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.jsx'],
    fallback: {
      "os": false,
      "path": false,
      "crypto": false,
    }
  },
  module: {
    rules: [
      {
        test: /\.m?(js|jsx)?$/,
        type: "javascript/auto",
        exclude: /node_modules/,
        resolve: {
          extensions: [".js", ".jsx"],
          fullySpecified: false,
        },
        use: ["babel-loader"],
      },
      {
        test: /\.(ts|tsx)?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/,
        exclude: /node_modules/,
        use: ["style-loader"],
      },
    ],
  },
  plugins: [
    new Dotenv(),
  ],
};

export default data;
