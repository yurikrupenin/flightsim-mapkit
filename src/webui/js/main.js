var map = new ol.Map({
    target: 'map',
    layers: [
      new ol.layer.Tile({
        source: new ol.source.OSM()
      })
    ],
    view: new ol.View({
      center: ol.proj.fromLonLat([37.41, 8.82]),
      zoom: 14
    })
  });

  var vectorSource = new ol.source.Vector({});

  var markersLayer = new ol.layer.Vector({
      source: vectorSource,
  });

  var gpxTrackLayer = new ol.layer.Vector({
    source: new ol.source.Vector({
    }),
  });


  var defaultPosition = new ol.geom.Point(
      ol.proj.fromLonLat([37.41, 8.82])
  );

  var markerStyle = new ol.style.Style({
    image: new ol.style.Circle({
        radius: 7,
        fill: new ol.style.Fill({color: 'black'}),
        stroke: new ol.style.Stroke({
            color: 'white', width: 2,
        })
    })
  });


  var markerFeature = new ol.Feature({
      geometry: defaultPosition,
      name: "Position Marker",
  });

  var gpxTrackStyle = new ol.style.Style({
    stroke: new ol.style.Stroke({
      width: 4,
      color: [255, 0, 0, 0.8]
    })
  });

  var centerOnPosition = true;


  function initialize() {
    map.addLayer(markersLayer);
    map.addLayer(gpxTrackLayer);

    vectorSource.addFeature(markerFeature);
    markerFeature.setStyle(markerStyle);

    console.log("Initialized!");
  }

  function drawRoute(jsonWaypoints) {
    gpxTrackLayer.getSource().clear();

    var jsondata = JSON.parse(jsonWaypoints);
    var route = new ol.geom.LineString(jsondata)
      .transform('EPSG:4326', 'EPSG:3857');

    var routeFeature = new ol.Feature({
      type: 'route',
      geometry: route
    });

    routeFeature.setStyle(gpxTrackStyle);

    gpxTrackLayer.getSource().addFeature(routeFeature);

    if (!centerOnPosition) {
      map.getView().fit(
        gpxTrackLayer.getSource().getExtent(), map.getSize(),
        {padding: [120, 55, 55, 55]}
      );
    }
  }


  var lastUpdate = 0;

  function updateCoords(lat, lon, alt) {
    markerFeature.setGeometry(new ol.geom.Point(ol.proj.fromLonLat([lon, lat])));

    // Center position only once in a second:
    // doing so every frame interrupts input events and prevents zoom
    if (centerOnPosition) {
      if ((new Date() - lastUpdate) > 1000) {
        map.getView().setCenter(ol.proj.fromLonLat([lon, lat]));
        lastUpdate = new Date();
      }
    }
  }
