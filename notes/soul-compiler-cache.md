A non-JUCE version of <https://github.com/soul-lang/SOUL/blob/master/include/soul/patch/helper_classes/soul_patch_CompilerCacheFolder.h>.

```cpp
struct CompilerCache: soul::patch::RefCountHelper<soul::patch::CompilerCache, CompilerCache>  {
    std::mutex mutex;
    std::filesystem::path path;

    CompilerCache(std::filesystem::path path): path(path) {
        std::filesystem::create_directory(path);
    }

    void storeItemInCache(const char* key, const void* source, uint64_t sourceSize) override {
        std::scoped_lock lock(mutex);
        std::ofstream file(this->path / key, std::ifstream::out | std::ifstream::binary);
        file.write((char*) source, sourceSize);
    }

    uint64_t readItemFromCache(const char* key, void* destination, uint64_t destinationSize) override {
        std::scoped_lock lock(mutex);
        std::ifstream file(this->path / key, std::ifstream::in | std::ifstream::binary);
        if (!file.is_open())
            return 0;
        file.seekg(0, file.end);
        uint64_t fileSize = file.tellg();
        if (fileSize == 0)
            return 0;
        if (destination == nullptr || destinationSize < fileSize)
            return fileSize;
        file.seekg(0, file.beg);
        file.read((char*) destination, destinationSize);
        return file.peek() == EOF ? fileSize : 0;
    }
};
```

It compiles and seems to work, but my patch isn't big enough to see any benefits.
I'd rather not introduce a potential source of error for now.
