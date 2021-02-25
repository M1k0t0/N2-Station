const SWPrecacheWebpackPlugin = require('sw-precache-webpack-plugin') // need to edit it's source code

module.exports = {
  "publicPath": '/',
  "configureWebpack": {
    plugins: [
      new SWPrecacheWebpackPlugin({
        cacheId: 'v1',
        filename: 'sw.js',
        staticFileGlobs: ['dist/**/*.{js,html,css,png,eot,svg,ttf,woff,woff2}'],
        minify: false,
        stripPrefix: 'dist/'
      })
    ]
  },
  "transpileDependencies": [
    "vuetify"
  ]
}