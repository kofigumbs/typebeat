mergeInto(LibraryManager.library, {
  typebeat_update: function(id, method, value) {
    globalThis.update(id, UTF8ToString(method), value);
  },
});
