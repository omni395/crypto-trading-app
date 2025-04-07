module.exports = {
  devServer: {
    port: 8080,
    hot: false,
    liveReload: true,
  },
  publicPath: '/',
  outputDir: '../static',
  assetsDir: '',
  indexPath: 'index.html',
  pages: {
    index: {
      entry: 'src/main.js',
      template: 'public/index.html',
      filename: 'index.html',
    },
  },
  chainWebpack: (config) => {
    config.plugin('define').tap((args) => {
      args[0]['process.env'] = JSON.stringify({
        NODE_ENV: process.env.NODE_ENV || 'development',
        VUE_APP_ENV: 'browser',
      });
      args[0]['__VUE_PROD_HYDRATION_MISMATCH_DETAILS__'] = false;
      return args;
    });
  },
};