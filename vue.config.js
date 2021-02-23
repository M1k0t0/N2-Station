const SWPrecacheWebpackPlugin = require('sw-precache-webpack-plugin')

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