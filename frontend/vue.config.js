module.exports = {
  devServer: {
    port: 8080,
    hot: false,
    liveReload: true
    //client: {
    //  webSocketURL: 'ws://127.0.0.1:8080/ws'
    //}
  },
  publicPath: '/',
  outputDir: '../static',
  assetsDir: '',
  indexPath: 'index.html',
  pages: {
    index: {
      entry: 'src/main.js',
      template: 'public/index.html',
      filename: 'index.html'
    }
  },
  chainWebpack: (config) => {
    config.plugin('define').tap((args) => {
      args[0]['__VUE_PROD_HYDRATION_MISMATCH_DETAILS__'] = false;
      return args;
    });
  }
};