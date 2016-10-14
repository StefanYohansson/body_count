$(document).ready(main);

var sources = [{ id: 0, name: 'No Source' }];

function main() {
  populateSourceSelect(populateDeads);
  initListeners();
}

function populateSourceSelect(callback) {
  var template = _.template($('#source-option-html').html());
  $.ajax({
    url: '/api/v1/sources'
  }).done(function(result) {
    result.forEach(function(source) {
      $('#select-source').append(template({ "source": source }));
    });
    callback();
    // @TODO: remove global assignment
    sources = sources.concat(result);
    sources = _.uniq(sources, 'id');
  });
}

function changeSource() {
  var self = $(this);
  var value = self.val();
  var source = sources.filter(function(i) { return i.id == value; });
  populateDeads(source[0]);
}

function populateDeads(source) {
  if (!source) {
    source = { id: 0, name: 'No Source' };
  }

  var templateHeader = _.template($('#deads-header-html').html());
  var templateList = _.template($('#deads-list-html').html());

  $.ajax({
    url: '/api/v1/deads'
  }).done(function(result) {
    var deads = result.filter(function(dead) {
      return dead.source == source.name;
    });
    $('#deads-header').html(templateHeader({ "source": source }));
    $('#deads-list').html(templateList({ "deads": deads }));
  });
}

function initListeners() {
  $('#select-source').on('change', changeSource);
}

function initMap() {
  // Specify features and elements to define styles.
  var styleArray = [
    {
      featureType: "all",
      stylers: [
        { saturation: -80 }
      ]
    },{
      featureType: "road.arterial",
      elementType: "geometry",
      stylers: [
        { hue: "#00ffee" },
              { saturation: 50 }
      ]
    },{
      featureType: "poi.business",
      elementType: "labels",
      stylers: [
        { visibility: "off" }
      ]
    }
  ];

  // Create a map object and specify the DOM element for display.
  var map = new google.maps.Map(document.getElementById('map'), {
          center: {lat: -34.397, lng: 150.644},
    scrollwheel: false,
    // Apply the map style array to the map.
    styles: styleArray,
    zoom: 8
  });
}
