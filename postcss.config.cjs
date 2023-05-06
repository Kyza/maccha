module.exports = {
	plugins: [
		// require("postcss-scrollbar")({ edgeAutohide: true }),
		require("autoprefixer"),
		require("postcss-mixins"),
		require("postcss-nested"),
		require("postcss-preset-env"),
	],
};
