<html>
	<head>
		<title>ACLED</title>
		<script defer src="/dart/main.dart" type="application/dart"></script>
		<script defer src="packages/browser/dart.js"></script>
        <link rel="stylesheet" type="text/css" href="/static/styles/normalize.css">
		<link rel="stylesheet" type="text/css" href="/static/styles/main.css">
	</head>
	<body>
        <h1>ACELD.rs</h1>
		<br />
		<!-- <img class="map" src="/static/images/africa_map_md.png"> -->
      
        <h3> Country List </h3>
        <table class="acled-main">
            <thead>
                <tr>
                    <th>Name</th>
                </tr>
            </thead>
            <tbody>
            {{#countries}}
            <tr>
                <td><b><a href="country/{{ link }}">{{ name }}</a></b></td>
            </tr>
            {{/countries}}
            </tbody>
        </table>
	</body>
</html>
