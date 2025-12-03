document.addEventListener('DOMContentLoaded', function() {
    const sortOrder = document.getElementById('sort-order');
    const targetWrapper = document.getElementById('sort-target-wrapper');
    const prefix = document.getElementById('prefix');
    const prefixWrapper = document.getElementById('prefix-wrapper');
    const rangeEndWrapper = document.getElementById('range-end-wrapper');
    const queryAll = document.getElementById('query-all');
    const keyWrapper = document.getElementById('key-wrapper');
    const keyInput = document.getElementById('key');

    // 初始检查
    if (sortOrder.value !== 'none') {
        targetWrapper.style.display = 'block';
    }
    if (prefix.checked) {
        rangeEndWrapper.style.display = 'none';
    }
    if (queryAll.checked) {
        keyWrapper.style.display = 'none';
        prefixWrapper.style.display = 'none';
        // 把key设置非必填
        keyInput.removeAttribute('required');
        // 那 range_end 也隐藏
        rangeEndWrapper.style.display = 'none';
    }

    // 如果启用查询所有 隐藏键输入
    queryAll.addEventListener('change', function() {
        if (this.checked) {
            keyWrapper.style.display = 'none';
            prefixWrapper.style.display = 'none';
            // 把key设置非必填
            keyInput.removeAttribute('required');
            // 那 range_end 也隐藏
            rangeEndWrapper.style.display = 'none';
        } else {
            keyWrapper.style.display = 'block';
            prefixWrapper.style.display = 'block';
            // 把key设置为必填
            keyInput.setAttribute('required', 'required');
            rangeEndWrapper.style.display = 'block';
        }
    });

    // 如果启用前缀查询 隐藏范围结束
    prefix.addEventListener('change', function() {
        if (this.checked) {
            rangeEndWrapper.style.display = 'none';
        } else {
            rangeEndWrapper.style.display = 'block';
        }
    });

    // 监听变化
    sortOrder.addEventListener('change', function() {
        if (this.value !== 'none') {
            targetWrapper.style.display = 'block';
        } else {
            targetWrapper.style.display = 'none';
        }
    });

    // 让前端提交checkbox时统一将on/off转换true/false
    const form = document.getElementById('range-form');
    form.addEventListener('submit', function(_event) {
        const checkboxes = document.querySelectorAll('input[type="checkbox"]');
        checkboxes.forEach(function(checkbox) {
            checkbox.value = checkbox.checked;
        });
    });

    form.addEventListener('reset', function(_event) {
        window.location.href = '/';
    });
});