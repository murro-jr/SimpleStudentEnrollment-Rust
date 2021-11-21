// Initializes the content within the document
function init() {
    document.getElementsByTagName('table')[0].style.visibility = 'hidden';
    document.getElementById('saveButton').style.display = '';
    document.getElementById('updateButton').style.display = 'none';
  
    clearFields();
  }
  
  // Resets the value of input fields to empty and makes sure that
  // inputId field is not locked or disabled
  function clearFields() {
    document.getElementById('inputName').value = '';
    document.getElementById('inputId').value = '';
    document.getElementById('inputYearLevel').value = '';
  
    document.getElementById('inputId').disabled = false;
  }
  
  function updateStudent(student) {
    let display = document.getElementById('saveButton').style.display;
  
    if (document.getElementById('updateButton').style.display == 'none') {
      if (document.getElementById('saveButton').style.display == '') {
  
        // Hide Save button and display Update button
        document.getElementById('updateButton').style.display = '';
        document.getElementById('saveButton').style.display = 'none';
  
        // Set value of input fields, disable editing of student ID
        document.getElementById('inputName').value = student.name;
        document.getElementById('inputYearLevel').value = student.level;
  
        var inputId = document.getElementById('inputId');
        inputId.value = student.id;
        inputId.disabled = true;
      }
    }
  }
  
  // @desc	Send POST request to save or register a new student
  // @route	POST /students
  function saveInfo() {
    var name = document.getElementById('inputName').value;
    var id = document.getElementById('inputId').value;
    var level = document.getElementById('inputYearLevel').value;
  
    let student = {
      name: name,
      id: id,
      level: level
    };
  
    var xhr = new XMLHttpRequest();
    xhr.open("POST", "/students", true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.setRequestHeader('X-Auth-Token', '123.exp.signature'); // TODO: Get from file
    xhr.send(JSON.stringify(student));
  
    if (document.getElementsByTagName('table')[0].style.visibility == 'visible') {
      showStudents();
    }
  }
  
  // @desc	Send PUT request to update and existing student
  // @route	PUT /students
  function updateInfo() {
    var name = document.getElementById('inputName').value;
    var id = document.getElementById('inputId').value;
    var level = document.getElementById('inputYearLevel').value;
  
    let student = {
      name: name,
      id: id,
      level: level
    };
  
    var xhr = new XMLHttpRequest();
    xhr.open("PUT", "/students/" + id, true);
    xhr.setRequestHeader('Content-Type', 'application/json');
    xhr.setRequestHeader('X-Auth-Token', '123.exp.signature'); // TODO: Get from file
    xhr.send(JSON.stringify(student));
  
    if (document.getElementsByTagName('table')[0].style.visibility == 'visible') {
      showStudents();
    }
  
    if (document.getElementById('saveButton').style.display == 'none') {
      // Hide Update button and display Save button
      document.getElementById('saveButton').style.display = '';
      document.getElementById('updateButton').style.display = 'none';
    }
  
    clearFields();
  }
  
  // @desc	Send DELETE request to remove a specific student info
  // @route	DELETE /students
  function deleteStudent(id) {
    var xhr = new XMLHttpRequest();
    xhr.open("DELETE", "/students/" + id, true);
    xhr.setRequestHeader('X-Auth-Token', '123.exp.signature'); // TODO: Get from file
    xhr.send(null);
  
    if (document.getElementsByTagName('table')[0].style.visibility == 'visible') {
      showStudents();
    }
  }
  
  // @desc	Send GET request to retrieve info of all students
  // @route	GET /students
  function showStudents() {
    var xhr = new XMLHttpRequest();
  
    xhr.onreadystatechange = function() {
      if (this.readyState != 4) return;
  
      if (this.status == 200) {
        var data = JSON.parse(this.responseText);
  
        // Check if we have an empty list of student information.
        // We don't want to display the table if it's empty.
        if (data.length <= 0) {
          if (document.getElementsByTagName('table')[0].style.visibility == 'visible') {
            document.getElementsByTagName('table')[0].style.visibility = 'hidden';
          }
  
          return;
        }
  
  
        // If the list is not empty, then we have to refresh the table so we don't append
        // immediately all data
        Array.prototype.slice.call(document.getElementsByClassName('studentInfo')).forEach(
          function(elem) {
            elem.remove();
          }
        );
  
        // Display the table
        document.getElementsByTagName('table')[0].style.visibility = 'visible';
  
        data.forEach(function(student) {
          var node = document.createElement("tr");
          node.className = "studentInfo";
  
          var id = document.createElement("td");
          var name = document.createElement("td");
          var level = document.createElement("td");
          var action = document.createElement("td");
          var updateButton = document.createElement("button");
          updateButton.appendChild(document.createTextNode("Update"));
  
          // User clicks Update, call updateStudent
          updateButton.addEventListener("click", function() {
            updateStudent(student);
          });
  
          var deleteButton = document.createElement("button");
          deleteButton.appendChild(document.createTextNode("Delete"));
  
          // User clicks Delete, call deleteStudent
          deleteButton.addEventListener("click", function() {
            deleteStudent(student.id);
          });
  
          action.appendChild(updateButton);
          action.appendChild(deleteButton);
          id.appendChild(document.createTextNode(student.id));
          name.appendChild(document.createTextNode(student.name));
          level.appendChild(document.createTextNode(student.level));
  
          node.appendChild(id);
          node.appendChild(name);
          node.appendChild(level);
          node.appendChild(action);
  
          document.getElementById('studentTable').appendChild(node);
        });
      }
    }
  
    xhr.open("GET", "/students", true);
    xhr.setRequestHeader('X-Auth-Token', '123.exp.signature'); // TODO: Get from file
    xhr.send();
  }
  